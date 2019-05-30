<?php
	include("connect.php");

	$server_name = "localhost";
	$database_name = "attend";
	$username = "attend";
	$password = "attend";

	$tag_id = filter_input(INPUT_POST, "tag_id", FILTER_SANITIZE_NUMBER_INT);

	$conn = connect($server_name, $database_name, $username, $password);

	$sql_query = "SELECT attendee_id, tag_id FROM Attendee WHERE tag_id=:tag_id";

	try {
		$statement = $conn->prepare($sql_query);
		$statement->bindParam(":tag_id", $tag_id, PDO::PARAM_STR, 50);
		$statement->execute();

		$statement_result = $statement->fetch(PDO::FETCH_ASSOC);
	} catch(PDOException $e) {
		echo "\nError: " . $e->getMessage();
	}

	if($statement_result["tag_id"] == $tag_id) {
		$attendee_id = $statement_result["attendee_id"];
		$sql_query = "INSERT INTO Attendance (attendee_id) VALUES (:attendee_id)";
		
		try {
		$statement = $conn->prepare($sql_query);
		$statement->bindParam(":attendee_id", $attendee_id, PDO::PARAM_INT);
		$statement->execute();

		echo "OK";
		} catch(PDOException $e) {
		echo "\nError: " . $e->getMessage();
		}
	} else {
		$sql_query = "SELECT tag_id FROM Tag WHERE tag_id=:tag_id";

		try {
		$statement = $conn->prepare($sql_query);
		$statement->bindParam(":tag_id", $tag_id, PDO::PARAM_STR, 50);
		$statement->execute();

		$statement_result = $statement->fetch(PDO::FETCH_ASSOC);

		if($statement_result["tag_id"] != $tag_id) {
			$sql_query = "INSERT INTO Tag (tag_id) VALUES (:tag_id)";

			try {
				$statement = $conn->prepare($sql_query);
				$statement->bindParam(":tag_id", $tag_id, PDO::PARAM_STR, 50);
				$statement->execute();
			} catch(PDOException $e) {
				echo "\nError: " . $e->getMessage();
			}
		}
			echo "In System Not An Attendee";
		} catch(PDOException $e) {
			echo "\nError: " . $e->getMessage();
		}
	}

	$conn = null;
?>
