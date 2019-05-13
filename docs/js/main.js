$(document).ready(function() {

  $('.nav-button').css('z-index', -1);
  $('.nav-button').css('margin-left', 0);
  $('.nav-button').css('margin-top', 0);
  $('.nav-button').css('width', '96px');


  var number = 0;

  $('nav #container img').css('cursor', 'pointer');
  $('header').css('margin-top', '0px');

  $('#logo').click(function() {
    if (number == 0) {
      if ($(window).width() < 730) {
        $('header').animate({
          marginTop: '475px'
        });
        $('#ide').animate({
          marginTop: '100px'
        }, 400);
        $('#ide').animate({
            paddingRight:'36%',
            paddingLeft:'30%',
            marginLeft:'-30%'

        }, 400);
      /*  $('#ide').animate({
          width:'75%'
        }, 300);*/

        $('#order').animate({
          marginTop: '225px'
        }, 400);
        $('#order').animate({
          paddingRight:'36%',
          paddingLeft:'30%',
          marginLeft:'-30%'
        }, 400);


        $('#about').animate({
          marginTop: '350px'
        }, 400);
        $('#about').animate({
          paddingRight:'36%',
          paddingLeft:'30%',
          marginLeft:'-30%'
        }, 400);


        $('#contact').animate({
          marginTop: '475px'
        }, 400);
        $('#contact').animate({
          paddingRight:'36%',
          paddingLeft:'30%',
          marginLeft:'-30%'
        }, 400, function() {

          $('.nav-button').css('z-index',0);
        });



      } else {
        $('#ide').animate({
          marginLeft: '-300px'
        }, 400);

        $('#order').animate({
          marginLeft: '-150px'
        }, 400);

        $('#about').animate({
          marginLeft: '150px'
        }, 400);

        $('#contact').animate({
          marginLeft: '300px'
        }, 400, function() {
          $('.nav-button').css('z-index',0);
        });
      }

      number = 1;
    } else {


      $('.nav-button').css('z-index',-1);
      $('#ide').animate({
        width: '96px'
      }, 300);
      $('#ide').animate({
        paddingRight:'0',
        paddingLeft:'0',
        marginLeft:'0'
      }, 400);
      $('#ide').animate({
        marginTop: '0px'
      }, 300);


      $('#order').animate({
        width: '96px'
      }, 300);
      $('#order').animate({
        paddingRight:'0',
        paddingLeft:'0',
        marginLeft:'0'
      }, 400);
      $('#order').animate({
        marginTop: '0px'
      }, 300);


      $('#about').animate({
        width: '96px'
      }, 300);
      $('#about').animate({
        paddingRight:'0',
        paddingLeft:'0',
        marginLeft:'0'
      }, 400);
      $('#about').animate({
        marginTop: '0px'
      }, 300);


      $('#contact').animate({
        width: '96px'
      }, 300);
      $('#contact').animate({
        paddingRight:'0',
        paddingLeft:'0',
        marginLeft:'0'
      }, 400);
      $('#contact').animate({
        marginTop: '0px'
      }, 300, function() {
        $('header').animate({
          marginTop: '0px'
        }, 400);

      });

      number = 0;
    }
  });
  $('#twitter').hover(function(){
    $('#twitter').css('width','60px');
    $('#twitter').css('height','60px');

  });
});
