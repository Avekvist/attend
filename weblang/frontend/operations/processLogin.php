<?php
    session_start();

	include("connect.php");

	$server_name = "localhost";
	$database_name = "attend";
	$database_username = "attend";
	$database_password = "attend";

    $username = filter_input(INPUT_POST, "username", FILTER_SANITIZE_STRING);
    $password = filter_input(INPUT_POST, "password", FILTER_SANITIZE_STRING);

    $conn = connect($server_name, $database_name, $database_username, $database_password);

    $sql_query = "SELECT Username, Password, IsAdmin FROM Teacher WHERE Username=:username";

    try {
        $statement = $conn->prepare($sql_query);
        $statement->bindParam(":username", $username);
        $statement->execute();

        $result = $statement->fetch();

        if(password_verify($password, $result["Password"])) {
            //setcookie("user", $result["IsAdmin"], time() + (86400 * 30), "/");
            if($result["IsAdmin"]) {
                $_SESSION["user"] = $result["IsAdmin"];
                echo "<script>window.location.replace('http://192.168.30.12/~attend/');</script>";
            } else {
                echo "Not an admin";
            }
        } else {
            echo "Incorrect password";
        }
    } catch(PDOException $e) {
        echo "\nError: " . $e->getMessage();
        echo "Couldn't fetch users. ";
    }
?>
