<!doctype html>
<html lang="sv">
	<head>
		<meta charset="utf-8" />
		<meta name="description" content="Logga in för att kolla närvaro med Attend" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />

		<title>Logga in - Attend</title>
		<link rel="stylesheet" type="text/css" href="css/main.css" />
		<link href="https://fonts.googleapis.com/css?family=Oxygen+Mono" rel="stylesheet" />
	</head>
	<body>
		<div class="container">
			<?php
				include("navigation.php");
			?>
			<main>
				<section class="support">
					<h2>Logga in</h2>
					<form method="post" action="operations/processLogin.php">
						<p>Användarnamn: </p><p><input type="text" name="username" /></p>
						<p>Lösenord: </p><p><input type="password" name="password" /></p>
						<input type="submit" value="Logga in" />
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