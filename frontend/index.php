<!doctype html>
<html lang="sv">
	<head>
		<meta charset="utf-8" />
		<meta name="description" content="Attend, ett närvarosystem" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />
		
		<title>Attend</title>
		<link rel="stylesheet" type="text/css" href="css/main.css" />
		<link href="https://fonts.googleapis.com/css?family=Oxygen+Mono" rel="stylesheet">
	</head>
	<body>
		<div class="container">
			<header>
				<h1>Attend</h1>
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
						
						$todays_date = date("Y-m-d");

						$sql_query = "SELECT attendee_id, class_id, Date FROM Attendance ORDER BY Date DESC";

						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();

							$attendance_dates = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}

						$sql_query = "SELECT DISTINCT attendee_id FROM Attendance";

						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();

							$distinct_attendees = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}

						$todays_attendees = array();

						foreach($attendance_dates as $attendance_date) {
							foreach($distinct_attendees as $distinct_attendee) {
								$distinct_attendee_today = true;
								foreach($todays_attendees as $todays_attendee) {
									if(!($distinct_attendee["attendee_id"] == $attendance_date["attendee_id"] && $distinct_attendee["attendee_id"] != $todays_attendee && $attendance_date["attendee_id"] != $todays_attendee)) {
										$distinct_attendee_today = false;
									}
								}

								if($distinct_attendee_today) {
									$date = explode(" ", htmlspecialchars($attendance_date["Date"]))[0];
									if($todays_date == $date) {
										array_push($todays_attendees, $attendance_date["attendee_id"]);
									}
								}
							}
						}

						$sql_query = "SELECT attendee_id, Name FROM Attendee";

						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();

							$attendee_results = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}
					} else {
						echo "<script>window.location.replace('http://192.168.30.12/~attend/login.php');</script>";
					}
				?>

				<div class="container">
					<main>
						<section class="index">
							<h2>Dagens frånvaro</h2>
							<?php
								if($loggedIn) {
									$not_attending = array();

									foreach($attendee_results as $attendee_result) {
										$is_not_attending = true;
										foreach($todays_attendees as $todays_attendee) {
											if($attendee_result["attendee_id"] == $todays_attendee) {
												$is_not_attending = false;
											}
										}

										if($is_not_attending) {
											array_push($not_attending, $attendee_result["attendee_id"]);
										}
									}
									
									foreach($attendee_results as $attendee_result) {
										foreach($not_attending as $not_todays_attendee) {
											if($attendee_result["attendee_id"] == $not_todays_attendee) {
												echo "<hr>";
												echo "<a href='students.php?attendee_id=" . $attendee_result["attendee_id"] . "'>" . $attendee_result["Name"] . "</a>";
											}
										}
									}
								}
							?>
						</section>
						<section class="index">
							<h2>Dagens närvaro</h2>
							<?php
								if($loggedIn) {
									foreach($attendee_results as $attendee_result) {
										foreach($todays_attendees as $todays_attendee) {
											if($attendee_result["attendee_id"] == $todays_attendee) {
												echo "<hr>";
												echo "<a href='students.php?attendee_id=" . $attendee_result["attendee_id"] . "'>" . $attendee_result["Name"] . "</a>";
											}
										}
									}
								}
							?>
						</section>
					</main>
				</div>
			</main>
			<?php
				include("footer.php");
			?>
		</div>
		
		<script src="https://code.jquery.com/jquery-3.3.1.min.js"></script>
		<script src="js/navigation_button.js"></script>
	</body>
</html>
