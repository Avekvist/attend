<!doctype html>
<html lang="sv">
	<head>
		<meta charset="utf-8" />
		<meta name="description" content="Taggar inlagda i Attend" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />
		
		<title>Taggar - Attend</title>
		<link rel="stylesheet" type="text/css" href="css/main.css" />
		<link href="https://fonts.googleapis.com/css?family=Oxygen+Mono" rel="stylesheet" />
	</head>
	<body>
		<div class="container">
			<?php
				include("navigation.php");
			?>
			<main>
				<section class='tags'><h2>Taggar</h2>
				<?php
					if($loggedIn) {
						include("operations/connect.php");
					
						$server_name = "localhost";
						$database_name = "attend";
						$username = "attend";
						$password = "attend";
					
						$conn = connect($server_name, $database_name, $username, $password);
					
						$sql_query = "SELECT tag_id FROM Attendee";

						$tag_id = filter_input(INPUT_GET, "tag_id", FILTER_SANITIZE_NUMBER_INT);
					
						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();
					
							$attendee_tags = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}
					
						$sql_query = "SELECT tag_id, Date FROM Tag ORDER BY Date";
					
						try {
							$statement = $conn->prepare($sql_query);
							$statement->execute();
					
							$tag_tags = $statement->fetchAll();
						} catch(PDOException $e) {
							echo "\nError: " . $e->getMessage();
						}

						if($tag_id == 0) {
							$every_tag_is_consumed = false;
							foreach($tag_tags as $tag) {
								$can_print_tag = true;

								foreach($attendee_tags as $attendee_tag) {
									if($tag["tag_id"] == $attendee_tag["tag_id"]) {
										$can_print_tag = false;
										$every_tag_is_consumed = true;
										break;
									} else {
										$every_tag_is_consumed = false;
									}
								}

								if($can_print_tag) {
									echo "<hr>";
									echo "<a href='tags.php?tag_id=" . htmlspecialchars($tag["tag_id"]) . "'>" . htmlspecialchars($tag["tag_id"]) . " - " . htmlspecialchars($tag["Date"]) . "</a>";
								}
							}

							if($every_tag_is_consumed) {
								echo "<hr>";
								echo "<p>Alla taggar är kopplade till en elev. </p>";
							}
						} else {
							echo "
								<form method='post' action='operations/addAttendee.php'>
									<p>Tagg ID: <br><input type='text' name='tag_id' value='" . htmlspecialchars($tag_id) . "' /></p>
									<p>Namn: <br><input type='text' name='attendee_name' /></p>
									<input type='submit' value='Lägg till' />
								</form>
							";
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
