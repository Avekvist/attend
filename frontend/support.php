<!doctype html>
<html lang="sv">
	<head>
		<meta charset="utf-8" />
		<meta name="description" content="Support för Attend" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />

		<title>Support - Attend</title>
		<link rel="stylesheet" type="text/css" href="css/main.css" />
		<link href="https://fonts.googleapis.com/css?family=Oxygen+Mono" rel="stylesheet" />
	</head>
	<body>
		<div class="container">
			<header>
				<h1>Support</h1>
			</header>
			<?php
				include("navigation.php");
			?>
			<main>
				<section class="support">
					<h2>Skicka en fråga</h2>
					<form method="post" action="operations/sendInquiry.php">
						<p>E-post: <input type="text" name="email" /></p>
						<section class="inquiry-type">
							<p>Typ av fråga: <br>
								<input type="radio" name="inquiry-type" value="problem" checked> Ett problem<br>
								<input type="radio" name="inquiry-type" value="function"> En ny funktion<br>
								<input type="radio" name="inquiry-type" value="other"> Annat
							</p>
						</section>
						<p>Fråga: <br><textarea name="inquiry" cols="20" rows="5"></textarea></p>
						<input type="submit" value="Skicka" />
					</form>
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
