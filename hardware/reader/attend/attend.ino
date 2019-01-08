#include <SPI.h>
#include <RFID.h>

#include <ESP8266WiFi.h>
#include <WiFiClient.h> 
#include <ESP8266WebServer.h>
#include <ESP8266HTTPClient.h>

struct Colour {
  int red;
  int green;
  int blue;
  
  Colour(int tRed, int tGreen, int tBlue) {
    red = tRed;
    green = tGreen;
    blue = tBlue;
  }
};

Colour none(0, 0, 0);
Colour red(1023, 0, 0);
Colour green(0, 1023, 0);
Colour blue(0, 0, 1023);
Colour yellow(780, 1000, 0);

const char *ssid[2] = { "tekniklabbet", "TeliaGatewayA4-B1-E9-7F-36-9B" };
const char *password[2] = { "tekniklabbet", "EB339ADAA0" };
const char *tryText[2] = { "in School", "at Home" };

const char *host = "192.168.30.12";
const char *requestDestination = "http://192.168.30.12/~attend/operations/handleTag.php";

enum {
  RST_PIN = 0,
  SS_PIN = 2,
  RED_PIN = 16,
  GREEN_PIN = 5,
  BLUE_PIN = 4,
};

RFID rfid(SS_PIN, RST_PIN);  // Create rfid instance.

void setup() {
  Serial.begin(9600);   // Initialize serial communications with the PC
  SPI.begin();          // Init SPI bus
  rfid.init();          // Init RFID Entity Reader

  pinMode(RED_PIN, OUTPUT);
  pinMode(GREEN_PIN, OUTPUT);
  pinMode(BLUE_PIN, OUTPUT);

  Serial.println("Initialising Attend. ");
}

void loop() {
  setColour(none);
  connectWiFi();
  handleTag();
}

void connectWiFi() {
  if(WiFi.status() != WL_CONNECTED) {
    WiFi.mode(WIFI_OFF);
    delay(1000);
    WiFi.mode(WIFI_STA);

    Serial.print("\nTries to connect to WiFi");
    for(int i = 0; i < 2; i++) {
      WiFi.begin(ssid[i], password[i]);
      
      setColour(yellow);
      
      Serial.print("\nTries ");
      Serial.println(tryText[i]);
      
      for(int tries = 0; tries < 30 && WiFi.status() != WL_CONNECTED; tries++) {
        Serial.print(".");
        delay(500);
      }

      Serial.println();
      
      if(WiFi.status() == WL_CONNECTED) {
        Serial.print("\nConnected to ");
        Serial.println(ssid[i]);
        Serial.print("My IP is: ");
        Serial.println(WiFi.localIP());
        return;
      }
    }
    
    setColour(red);
    Serial.println("*Cries*");
  }
}

void handleTag() {
  if (rfid.isCard()) {
    // Select one of the RFID Entities
    if (rfid.readCardSerial()) {
      Serial.println("\nRFID Entity found. ");
      
      String resultNumber = "";
      for(int i = 0; i < 5; i++) {
        resultNumber += String(rfid.serNum[i]);
      }
      
      Serial.print("RFID Entity ID: ");
      Serial.println(resultNumber);

      Serial.print("\nSending the RFID Entity ID...");
      
      setColour(yellow);
      String result = sendInformation(resultNumber);
      
      Serial.print("\nServer responded with: ");
      Serial.println(result);
      
      Colour resultColour = result == "OK" ? green : result == "In System Not An Attendee" ? blue : red;
      setColour(resultColour);
      delay(2000); 
    }
  }
}

void setColour(Colour colour) {
  analogWrite(RED_PIN, 1023 - colour.red);
  analogWrite(GREEN_PIN, 1023 - colour.green);
  analogWrite(BLUE_PIN, 1023 - colour.blue);
}

String sendInformation(String data) {
  HTTPClient http;

  String postData = "tag_id=" + data;
  
  http.begin(requestDestination);
  http.addHeader("Content-Type", "application/x-www-form-urlencoded");

  int httpCode = http.POST(postData);
  String result = http.getString();
  
  http.end();

  return result;
}
