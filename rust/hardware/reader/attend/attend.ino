#include <SPI.h>
#include <RFID.h>

#include <ESP8266WiFi.h>
#include <WiFiClient.h> 
#include <ESP8266WebServer.h>
#include <ESP8266HTTPClient.h>

// A Pin enum to group all the pins
enum struct Pin : uint8_t {
  // Reset pin for the RFID Reader. 
  reset = 0,

  // Slave Select pin for the RFID Reader. 
  slaveSelect = 2,

  // Red color pin for the multicolor LED. 
  red = 16,

  // Green color pin for the multicolor LED. 
  green = 5,

  // Blue color pin for the multicolor LED. 
  blue = 4,
};

struct Color {
private:
  short red, green, blue;
  
public:
  Color(short red, short green, short blue) {
    this->red = red;
    this->green = green;
    this->blue = blue;
  }

  // Getters for the colors. 
  short getRed() { return red; }
  short getGreen() { return green; }
  short getBlue() { return blue; }
};

// Define the different colors used
// by the LED on this device. 
static Color none(0, 0, 0);
static Color red(1023, 0, 0);
static Color green(0, 1023, 0);
static Color blue(0, 0, 1023);
static Color yellow(780, 1000, 0);

class LED {
private:
  Color *color;
  Pin redPin, greenPin, bluePin;
  bool enabled;
  
  // Updates the color on the LED. 
  // If the LED is disabled, nothing
  // will happen. 
  void update() {
    if(enabled) {
      analogWrite((uint8_t) redPin, 1023 - this->color->getRed());
      analogWrite((uint8_t) greenPin, 1023 - this->color->getGreen());
      analogWrite((uint8_t) bluePin, 1023 - this->color->getBlue());
    }
  }
public:
  LED(Pin redPin, Pin greenPin, Pin bluePin) {
    // Assign the pins to the pins of a new LED object. 
    this->redPin = redPin;
    this->greenPin = greenPin;
    this->bluePin = bluePin;
    this->enabled = true;
    
    // Set the pin mode for the multicolor LED pins. 
    pinMode((uint8_t) this->redPin, OUTPUT);
    pinMode((uint8_t) this->greenPin, OUTPUT);
    pinMode((uint8_t) this->bluePin, OUTPUT);
  }

  // Sets the color on the LED. 
  void setColor(Color color) {
    *this->color = color;
    this->update();
  }

  // Enables the LED. 
  void enable() {
    this->enabled = true;
    this->update();
  }

  // Disables the LED. 
  void disable() {
    enabled = false;
    analogWrite((uint8_t) redPin, 1023);
    analogWrite((uint8_t) greenPin, 1023);
    analogWrite((uint8_t) bluePin, 1023);
  }
};

class Network {
private:
  const char *ssid, *password;
  
public:
  enum ResultType : uint8_t {
    SUCCESS,
    ALREADY_CONNECTED,
    COULD_NOT_CONNECT,
  };
  
  Network(const char* ssid, const char* password) : ssid(ssid), password(password) {}

  // Tries to connect to the WiFi specified
  // Returns if it succeeded or not. 
  ResultType connect(LED* led) {
    // Check if the device is connected to the WiFi. 
    // Proceed with trying to reconnect if it isn't. 
    if(WiFi.status() != WL_CONNECTED) {
      // Reinitializes the WiFi chip. 
      WiFi.mode(WIFI_OFF);
      delay(1000);
      WiFi.mode(WIFI_STA);
  
      WiFi.begin(ssid, password);
      
      led->setColor(yellow);

      // Tries 30 times with 0.5 seconds inbetween the tries
      // to connect to the WiFi network. 
      for(int tries = 0; tries < 30 && WiFi.status() != WL_CONNECTED; tries++) {
        delay(500);
      }
      
      if(WiFi.status() == WL_CONNECTED) {
        return SUCCESS;
      }
      
      return COULD_NOT_CONNECT;
    }

    return ALREADY_CONNECTED;
  }
};

class WebServer {
private:
  const char* requestDestination;
  
public:
  enum ResponseType : uint8_t {
    OK,
    IN_SYSTEM,
    ERROR,
  };
  
  WebServer(const char *requestDestination) : requestDestination(requestDestination) {}

  // Sends data to the requestDestination and 
  // returns the result. 
  ResponseType send(String data) {
    HTTPClient http;
    
    String postData = "tag_id=" + data;
    
    http.begin(this->requestDestination);
    http.addHeader("Content-Type", "application/x-www-form-urlencoded");
  
    int httpCode = http.POST(postData);
    String result = http.getString();
    
    http.end();

    if(result == "OK") {
      return OK;
    } else if(result == "In System Not An Attendee") {
      return IN_SYSTEM;
    } else {
      return ERROR;
    }
  }
};

class RFIDReader {
private:
  Pin slaveSelect, reset;
  RFID* rfid;
  
public:
  RFIDReader(Pin slaveSelect, Pin reset) {
    this->slaveSelect = slaveSelect;
    this->reset = reset;
    this->rfid = new RFID((uint8_t) slaveSelect, (uint8_t) reset);

    // Initialize the SPI Bus. 
    SPI.begin();
    
    // Initialize the low-level RFID Entity Reader. 
    rfid->init();
  }

  // Handle reading the RFID Entity and sending
  // the acquired data to a web server. 
  WebServer::ResponseType handle(LED* led, WebServer* server) {
    if (rfid->isCard()) {
      // Select one of the RFID Entities
      if (rfid->readCardSerial()) {
        String resultNumber = "";

        // Reads all the values from the serial number
        // and puts them into one unified String. 
        for(int i = 0; i < 5; i++) {
          resultNumber += String(rfid->serNum[i]);
        }
        
        led->setColor(yellow);
        return server->send(resultNumber);
      }
    }
  }
};

////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
/////////// Separator for library parts and executable parts ///////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////

// Allocate some memory for the
// required components.
Network* network;
WebServer* server;
RFIDReader* reader;
LED* led;

void setup() {
  // Define the required components. 
  led = new LED(Pin::red, Pin::green, Pin::blue);
  reader = new RFIDReader(Pin::slaveSelect, Pin::reset);
  server = new WebServer("http://192.168.30.12:8000/handle_tag");
  network = new Network("tekniklabbet", "tekniklabbet");
}

void loop() {
  // Check if it's connected to WiFi, try to connect if it isn't. 
  switch(network->connect(led)) {
    case Network::SUCCESS: 
    case Network::ALREADY_CONNECTED:
      // Reset the LED color. 
      led->setColor(none);
      
      // Read from the RFID device to see if there is any RFID Entity nearby. 
      // Match the result from the method and change the LED color accordingly. 
      switch(reader->handle(led, server)) {
        case WebServer::OK:
          led->setColor(green);
          delay(2000);
          break;
        case WebServer::IN_SYSTEM:
          led->setColor(blue);
          delay(2000);
          break;
        case WebServer::ERROR:
        default:
          led->setColor(red);
          delay(2000);
          break;
      }
      break;
    case Network::COULD_NOT_CONNECT:
    default:
      // Display with the LED that 
      // the connection failed. 
      led->setColor(red);
      break;
  }
}
