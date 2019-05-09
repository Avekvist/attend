$(document).ready(function(){

  var number = 0;



  $('#ide').css("margin-left","0px")
  $('#ide a').css("display","none")

  $('#order').css("margin-left","0px")
  $('#order a').css("display","none")

  $('#about').css("margin-left","0px")
  $('#about a').css("display","none")

  $('#contact').css("margin-left","0px")
  $('#contact a').css("display","none")





  $('#logo').click(function(){
    if(number==0){
      if($(window).width()<730){
        $('header').animate({
          marginTop:'475px'
        });

        $('#ide').animate({
          marginTop:'100px'
        },400);
        $('#ide').animate({
          marginLeft:'-30%'
        },400);
        $('#ide').animate({
          width:'75%'
        },300, function(){
          $('#ide a').css('display', "block");
        });

        $('#order').animate({
          marginTop:'225px'
        },400);
        $('#order').animate({
          marginLeft:'-30%'
        },400);
        $('#order').animate({
          width:'75%'
        },300, function(){
          $('#order a').css('display', "block");
        });

        $('#about').animate({
          marginTop:'350px'
        },400);
        $('#about').animate({
          marginLeft:'-30%'
        },400);
        $('#about').animate({
          width:'75%'
        },300, function(){
          $('#about a').css('display', "block");
        });

        $('#contact').animate({
          marginTop:'475px'
        },400);
        $('#contact').animate({
          marginLeft:'-30%'
        },400);
        $('#contact').animate({
          width:'75%'
        },300, function(){
          $('#contact a').css('display', "block");
        });



      }else{
        $('#ide').animate({
          marginLeft:'-300px'
        },400, function(){
          $('#ide a').css('display', "block");
        });

        $('#order').animate({
        marginLeft:'-150px'
        },400, function(){
        $('#order a').css('display', 'block');
        });

        $('#about').animate({
        marginLeft:'150px'
        },400, function(){
          $('#about a').css('display', 'block');
        });

        $('#contact').animate({
        marginLeft:'300px'
        },400, function(){
          $('#contact a').css('display', 'block');
        });

      }

      number=1;
    }else{
      if($(window).width()<730){
        $('#ide a').css('display','none');
        $('#ide').animate({
          width:'96px'
        },300);
        $('#ide').animate({
          marginLeft:'0%'
        },400);
        $('#ide').animate({
          marginTop:'0px'
        },300);

        $('#order a').css('display','none');
        $('#order').animate({
          width:'96px'
        },300);
        $('#order').animate({
          marginLeft:'0%'
        },400);
        $('#order').animate({
          marginTop:'0px'
        },300);

        $('#about a').css('display','none');
        $('#about').animate({
          width:'96px'
        },300);
        $('#about').animate({
          marginLeft:'0%'
        },400);
        $('#about').animate({
          marginTop:'0px'
        },300);

        $('#contact a').css('display','none');
        $('#contact').animate({
          width:'96px'
        },300);
        $('#contact').animate({
          marginLeft:'0%'
        },400);
        $('#contact').animate({
          marginTop:'0px'
        },300, function(){
          $('header').animate({
            marginTop:'0px'
          },400);

        });



      }else{

        $('#ide a').css('display', "none");
        $('#ide').animate({
          marginLeft:'0px'
        },400);

        $('#order a').css('display', 'none');
        $('#order').animate({
          marginLeft:'0px'
        },400);

        $('#about a').css('display', 'none');
        $('#about').animate({
        marginLeft:'0px'
        },400);

        $('#contact a').css('display', 'none');
        $('#contact').animate({
        marginLeft:'0px'
        },400);
      }

        number=0;
    }
  });











});
