<!doctype html>
<html>
	<head>
		<meta charset="utf-8" />
		<meta name="description" content="Lista över närvaron i Attend. " />
		<meta name="viewport" content="width=device-width, initial-scale=1" />

		<title>Närvaro - Attend</title>
		<link rel="stylesheet" type="text/css" href="css/main.css" />
		<link href="https://fonts.googleapis.com/css?family=Oxygen+Mono" rel="stylesheet" />
	</head>
	<body>
		<div class="container">
			<header>
				<h1>Närvaro</h1>
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

						$attendance_date = filter_input(INPUT_GET, "attendance_date", FILTER_SANITIZE_URL);

						$sql_query = "SELECT class_id, attendee_id, Date FROM Attendance ORDER BY Date DESC";

						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();

							$attendance_results = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}

						$sql_query = "SELECT attendee_id, Name FROM Attendee";

						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();

							$attendee_results = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}

						if($attendance_date == "") {
							$latest_date = "";
							foreach($attendance_results as $attendance_result) {
								$date = explode(" ", htmlspecialchars($attendance_result["Date"]));

								if($latest_date != $date[0]) {
									$latest_date = $date[0];

									echo "<a href='?attendance_date=" . $latest_date . "'><section><h2>" . $latest_date . "</h2></section></a>";
								}
							}
						} else {
							echo "<section class='index'><h2>" . htmlspecialchars($attendance_date) . "</h2>";
							foreach($attendance_results as $attendance_result) {
								$date = explode(" ", htmlspecialchars($attendance_result["Date"]));

								if($date[0] == $attendance_date) {
									foreach($attendee_results as $attendee_result) {
										if($attendance_result["attendee_id"] == $attendee_result["attendee_id"]) {
											echo "<hr>";
											echo "<a href='students.php?attendee_id=" . htmlspecialchars($attendee_result["attendee_id"]) . "'>" . $attendee_result["Name"] . " - " . $date[1] . "</a>";
										}
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
