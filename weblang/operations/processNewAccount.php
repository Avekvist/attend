<?php
	include("connect.php");

	$server_name = "localhost";
	$database_name = "attend";
	$database_username = "attend";
	$database_password = "attend";

    $name = filter_input(INPUT_POST, "name", FILTER_SANITIZE_STRING);
    $username = filter_input(INPUT_POST, "username", FILTER_SANITIZE_STRING);
    $password = password_hash(filter_input(INPUT_POST, "password", FILTER_SANITIZE_STRING), PASSWORD_DEFAULT);

    $conn = connect($server_name, $database_name, $database_username, $database_password);

    $sql_query = "SELECT Username FROM Teacher WHERE Username=:username";

    $can_continue = false;

    $success_code = -1;

    try {
        $statement = $conn->prepare($sql_query);
        $statement->bindParam(":username", $username);

        $result = $statement->fetchAll();
        
        if(sizeof($result) < 1) {
            $can_continue = true;
        } else {
            $success_code = 3;
        }
    } catch(PDOException $e) {
        echo "\nError: " . $e->getMessage();
        $success_code = 2;
    }

    if($can_continue) {
        $sql_query = "INSERT INTO Teacher (Name, Username, Password) VALUES (:name, :username, :password)";

        try {
            $statement = $conn->prepare($sql_query);

            $statement->bindParam(":name", $name);
            $statement->bindParam(":username", $username);
            $statement->bindParam(":password", $password);

            $statement->execute();
            $success_code = 0;
        } catch(PDOException $e) {
            echo "\nError: " . $e->getMessage();
            $success_code = 1;
        }
    }

    switch($success_code) {
        case 0: 
            echo "<h1>Success!</h1>";
            echo "<script>window.location.replace('http://192.168.30.12/~attend/login.php');</script>";
            break;
        case 1:
            echo "<h1>Failure</h1><p>Couldn't add user to database. </p>";
            break;
        case 2:
            echo "<h1>Failure</h1><p>Couldn't fetch current users. </p>";
            break;
        case 3:
            echo "<h1>Failure</h1><p>A user with that username already exists. </p>";
            break;
        default:
            echo "<h1>Unknown error</h1><p>An unknown error has occurred. Try again later</p>";
            break;
    }
?>
