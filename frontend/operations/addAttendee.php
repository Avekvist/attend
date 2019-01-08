<?php
    include("connect.php");

	$server_name = "localhost";
	$database_name = "attend";
	$username = "attend";
	$password = "attend";

    $conn = connect($server_name, $database_name, $username, $password);
    
    $attendee_name = filter_input(INPUT_POST, "attendee_name", FILTER_SANITIZE_STRING);
    $tag_id = filter_input(INPUT_POST, "tag_id", FILTER_SANITIZE_NUMBER_INT);

	$sql_query = "SELECT Name, tag_id FROM Attendee";

	try {
		$statement = $conn->prepare($sql_query);
		$statement->execute();

		$statement_result = $statement->fetchAll();
	} catch(PDOException $e) {
		echo "\nError: " . $e->getMessage();
    }

    $can_continue = true;
    foreach($statement_result as $result) {
        if($result["tag_id"] == $tag_id || $result["Name"] == $attendee_name) {
            $can_continue = false;
        }
    }

    if($can_continue) {
        $sql_query = "INSERT INTO Attendee (Name, tag_id) VALUES (:attendee_name, :tag_id)";

        try {
            $statement = $conn->prepare($sql_query);
            $statement->bindParam(":attendee_name", $attendee_name);
            $statement->bindParam(":tag_id", $tag_id);
            $statement->execute();

            echo "<script>window.location.replace('http://192.168.30.12/~attend/tags.php/');</script>";
        } catch(PDOException $e) {
            echo "\nError: " . $e->getMessage();
        }
    } else {
        echo "<script>window.location.replace('http://192.168.30.12/~attend/tags.php?tag_id='" . htmlspecialchars($tag_id) . " . ');</script>";
    }
?>
