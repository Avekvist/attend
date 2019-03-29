<!doctype html>
<html lang="sv">
	<head>
		<meta charset="utf-8" />
		<meta name="description" content="Lista över klasser" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />

		<title>Kurser - Attend</title>
		<link rel="stylesheet" type="text/css" href="css/main.css" />
		<link href="https://fonts.googleapis.com/css?family=Oxygen+Mono" rel="stylesheet" />
	</head>
	<body>
		<div class="container">
			<header>
				<h1>Kurser</h1>
			</header>
			<?php
				include("navigation.php");
			?>
			<main>
				<?php
					if($loggedIn) {
						include("operations/connect.php");
					
						$server_name = "localhost";
						$database_name = "attend";
						$username = "attend";
						$password = "attend";
					
						$conn = connect($server_name, $database_name, $username, $password);
					
						$class_id = filter_input(INPUT_GET, "class_id", FILTER_SANITIZE_NUMBER_INT);
					
						$sql_query = "SELECT Name, attendee_id FROM Attendee";
					
						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();
					
							$attendee_result = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}
					
						$sql_query = "SELECT attendee_id, class_id FROM AttendeeClassBridge";
					
						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();
					
							$bridge_result = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}
					
						$sql_query = "SELECT class_id, Name FROM Class";
					
						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();
					
							$class_result = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}
					
						$sql_query = "SELECT Name FROM Class WHERE class_id=:class_id";
					
						try {
							$statement = $conn->prepare($sql_query);
							$statement->bindParam(":class_id", $class_id, PDO::PARAM_INT);
							$statement->execute();
					
							$class_name = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}
				
						if($class_id == 0) {
							foreach($class_result as $c_result) {
								echo "<a href='?class_id=" . htmlspecialchars($c_result["class_id"]) . "'><section><h2>" . htmlspecialchars($c_result["Name"]) . "</h2>";
								echo "</section></a>";
							}
						} else {
							echo "<section class='index'><h2>" . htmlspecialchars($class_name[0]["Name"]) . "</h2>";
							
							foreach($attendee_result as $a_result) {
								foreach($bridge_result as $b_result) {
									if($a_result["attendee_id"] == $b_result["attendee_id"] && $class_id == $b_result["class_id"]) {
										echo "<hr>";
										echo "<a href='students.php?attendee_id=" . htmlspecialchars($a_result["attendee_id"]) . "'>" . htmlspecialchars($a_result["Name"]) . "</a>";
									}
								}
							}

							echo "</section>";
						}
					} else {
						echo "<script>window.location.replace('http://192.168.30.12/~attend/login.php');</script>";
					}
				?>
			</main>
			<?php
				include("footer.php");
			?>
		</div>
		
		<script src="https://code.jquery.com/jquery-3.3.1.min.js"></script>
		<script src="js/navigation_button.js"></script>
	</body>
</html>
