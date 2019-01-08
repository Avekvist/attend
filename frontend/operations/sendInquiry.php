<?php
    $email = filter_input(INPUT_POST, "email", FILTER_SANITIZE_EMAIL);
    $inquiry_type = filter_input(INPUT_POST, "inquiry-type", FILTER_DEFAULT);
    $inquiry = filter_input(INPUT_POST, "inquiry", FILTER_SANITIZE_STRING);

    switch($inquiry_type) {
        case "problem":
            $subject = "An issue. ";
            break;
        case "function":
            $subject = "New functionality. ";
            break;
        case "other": 
            $subject = "Something else... ";
            break;
    }
    
    $success = mail("hampus@avekvist.com", $subject, $inquiry, "From:" . $email);

    echo "<h1>";
    if($success) {
        echo "Successfully sent the inquiry. ";
    } else {
        echo "Something went wrong. ";
    }
    echo "</h1>";
?>
