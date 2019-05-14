$(document).ready(function() {

  $('.nav-button').css('z-index', -1);
  $('.nav-button').css('margin-left', 0);
  $('.nav-button').css('margin-top', 0);
  $('.nav-button').css('width', '96px');
  $('.nav-button').css('display', 'none');

  $('form').css('display', 'block');


  var menuNotActive = 0;

  $('nav #container img').css('cursor', 'pointer');
  $('header').css('margin-top', '0px');

  $('#logo').click(function() {
    if (menuNotActive == 0) {

      $('.nav-button').css('display', 'block');

      if ($(window).width() < 730) {
        $('header').animate({
          marginTop: '525px'
        });
        $('#ide').animate({
          marginTop: '150px'
        }, 300);
        $('#ide').animate({
          paddingRight: '36%',
          paddingLeft: '30%',
          marginLeft: '-30%'

        }, 300);

        $('#order').animate({
          marginTop: '275px'
        }, 300);
        $('#order').animate({
          paddingRight: '36%',
          paddingLeft: '30%',
          marginLeft: '-30%'
        }, 300);


        $('#about').animate({
          marginTop: '400px'
        }, 300);
        $('#about').animate({
          paddingRight: '36%',
          paddingLeft: '30%',
          marginLeft: '-30%'
        }, 300);


        $('#contact').animate({
          marginTop: '525px'
        }, 300);
        $('#contact').animate({
          paddingRight: '36%',
          paddingLeft: '30%',
          marginLeft: '-30%'
        }, 300, function() {

          $('.nav-button').css('z-index', 0);
        });



      } else {
        $('#ide').animate({
          marginLeft: '-300px'
        }, 300);

        $('#order').animate({
          marginLeft: '-150px'
        }, 300);

        $('#about').animate({
          marginLeft: '150px'
        }, 300);

        $('#contact').animate({
          marginLeft: '300px'
        }, 300, function() {
          $('.nav-button').css('z-index', 0);
        });
      }

      menuNotActive = 1;
      console.log(menuNotActive);
    } else {


      $('.nav-button').css('z-index', -1);
      $('#ide').animate({
        width: '96px'
      }, 300);
      $('#ide').animate({
        paddingRight: '0',
        paddingLeft: '0',
        marginLeft: '0'
      }, 400);
      $('#ide').animate({
        marginTop: '0px'
      }, 300);


      $('#order').animate({
        width: '96px'
      }, 300);
      $('#order').animate({
        paddingRight: '0',
        paddingLeft: '0',
        marginLeft: '0'
      }, 400);
      $('#order').animate({
        marginTop: '0px'
      }, 300);


      $('#about').animate({
        width: '96px'
      }, 300);
      $('#about').animate({
        paddingRight: '0',
        paddingLeft: '0',
        marginLeft: '0'
      }, 400);
      $('#about').animate({
        marginTop: '0px'
      }, 300);


      $('#contact').animate({
        width: '96px'
      }, 300);
      $('#contact').animate({
        paddingRight: '0',
        paddingLeft: '0',
        marginLeft: '0'
      }, 400);

      $('#contact').animate({
        marginTop: '0px'
      }, 300, function() {
        $('header').animate({
          marginTop: '0px'
        }, 400);

        $('.nav-button').css('display', 'none');

      });

      menuNotActive = 0;
      console.log(menuNotActive);
    }
  });


  $('#logo').mouseover(function() {
    $('#logo').animate({
      width: '120px',
      height: '120px',
      marginLeft: '-10'

    }, 200);
  });

  $('#logo').mouseleave(function() {
    if (menuNotActive == 0) {
      $('#logo').animate({
        width: '96px',
        height: '96px',
        marginLeft: '0'

      }, 200);

    }
  });
  /*$(document).on('scroll', function(){

    if($(document).scrollTop() > 15 && $('#logo').width()!=120 ){
      $('#logo').animate({
        width:'70px',
        height:'70px',
      },100);
    }else{
      if($(document).scrollTop() < 15){
        $('#logo').animate({
          width:'96px',
          height:'96px',
        },100);
      }
    }
  });*/


  var i = 0;

  function testInfo(phoneInput, re, textsvar) {
    var OK = re.exec(phoneInput);

    if (!OK) {
      $(textsvar).css("color", "red");
    } else {
      $(textsvar).css("color", "green");
      i++;
    }
  }



  $("#phone").keyup(function() {
    var re = /^[0]{1}[0-9]{6,15}$/;
    testInfo($('#phone').val(), re, "#phone");
  });


  $("#mail").keyup(function() {
    var rea = /^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\\.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    testInfo($('#mail').val(), rea, "#mail");
  });

  $("#firstName").keyup(function() {
    var check = /^[a-zA-Z]+$/;
    testInfo($('#firstName').val(), check, "#firstName");

  });

  $("#lastName").keyup(function() {
    var check2 = /^[a-zA-Z]+$/;
    testInfo($('#lastName').val(), check2, "#lastName");


  });



});
