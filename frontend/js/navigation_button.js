$(document).ready(function() {
    if($(window).width() > 960) {
        $(".navbar").css("display", "inherit");
    } else {
        $(".navbar").css("display", "none");
    }
});

$("#handle").on("click", function() {
    $(".navbar").animate({
        height: "toggle",
    }, 500, function() {});
});

$(window).resize(function() {
    if($(window).width() > 960) {
        $(".navbar").css("display", "inherit");
    } else {
        $(".navbar").css("display", "none");
    }
});
