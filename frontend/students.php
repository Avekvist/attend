<!doctype html>
<html lang="sv">
	<head>
		<meta charset="utf-8" />
		<meta name="description" content="En lista över alla studenter med deras senaste närvaro" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />

		<title>Elever - Attend</title>
		<link rel="stylesheet" type="text/css" href="css/main.css" />
		<link href="https://fonts.googleapis.com/css?family=Oxygen+Mono" rel="stylesheet" />
	</head>
	<body>
		<div class="container">
			<?php
				include("navigation.php");
			?>
			<main>
				<section class='students'>
				<?php
					if($loggedIn) {
						include("operations/connect.php");
					
						$server_name = "localhost";
						$database_name = "attend";
						$username = "attend";
						$password = "attend";
					
						$conn = connect($server_name, $database_name, $username, $password);
					
						$attendee_id = filter_input(INPUT_GET, "attendee_id", FILTER_SANITIZE_NUMBER_INT);
						
						if($attendee_id == 0) {
							$sql_query = "SELECT Name, attendee_id FROM Attendee";
					
							try {
								$statement = $conn->prepare($sql_query);
								$statement->execute();
					
								$attendee_result = $statement->fetchAll();
							} catch(PDOException $e) {
								echo "\nError: " . $e->getMessage();
							}
						} else {
							$sql_query = "SELECT Name FROM Attendee WHERE attendee_id=:attendee_id";
					
							try {
								$statement = $conn->prepare($sql_query);
								$statement->bindParam(":attendee_id", $attendee_id, PDO::PARAM_INT);
								$statement->execute();
					
								$attendee_name = $statement->fetchAll();
							} catch(PDOException $e) {
								echo "\nError: " . $e->getMessage();
							}
					
							$sql_query = "SELECT class_id, Name FROM Class";
					
							try {
								$statement = $conn->prepare($sql_query);
								$statement->execute();
					
								$class_results = $statement->fetchAll();
							} catch(PDOException $e) {
								echo "\nError: " . $e->getMessage();
							}
					
							$sql_query = "SELECT attendee_id, class_id FROM AttendeeClassBridge";
					
							try {
								$statement = $conn->prepare($sql_query);
								$statement->execute();
					
								$bridge_results = $statement->fetchAll();
							} catch(PDOException $e) {
								echo "\nError: " . $e->getMessage();
							}
					
							$attendees_classes = array();
					
							foreach($bridge_results as $bridge_result) {
								foreach($class_results as $class_result) {
									if($attendee_id == $bridge_result["attendee_id"] && $bridge_result["class_id"] == $class_result["class_id"]) {
										array_push($attendees_classes, $class_result["class_id"]);
									}
								}
							}
						}
				
						if($attendee_id == 0) {
							echo "<h2>Elever</h2>";
							foreach($attendee_result as $a_result) {
								echo "<hr>";
								echo "<a href='?attendee_id=" . htmlspecialchars($a_result["attendee_id"]) . "'>" . htmlspecialchars($a_result["Name"]) . "</a>";
							}
						} else {
							echo "<h2>" . htmlspecialchars($attendee_name[0]["Name"]) . "</h2>";
							foreach($attendees_classes as $attendees_class) {
								foreach($class_results as $class_result) {
									if($attendees_class == $class_result["class_id"]) {
										echo "<hr>";
										echo "<a href='classes.php?class_id=" . $class_result["class_id"] . "'>" . htmlspecialchars($class_result["Name"]) . "</a>";
									}
								}
							}
						}
					} else {
						echo "<script>window.location.replace('http://192.168.30.12/~attend/login.php');</script>";
					}
				?>
				</section>
			</main>
			<?php
				include("footer.php");
			?>
		</div>
		
		<script src="https://code.jquery.com/jquery-3.3.1.min.js"></script>
		<script src="js/navigation_button.js"></script>
	</body>
</html>
