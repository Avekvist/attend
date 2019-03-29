<?php
    session_start();
    $loggedIn = isset($_SESSION["user"]);
?>

<nav>
    <div class="navbar">
        <?php
            if($loggedIn) {
                echo "
                    <a href='index.php'>Hem</a>
                    <a href='attendance.php'>Närvaro</a>
                    <a href='students.php'>Elever</a>
                    <a href='classes.php'>Kurser</a>
                    <a href='tags.php'>Taggar</a>
                    <a href='support.php'>Support</a>
                    <a href='logout.php'>Logga ut</a>
                ";
            } else {
                echo "
                    <a href='login.php'>Logga in</a>
                    <a href='createAccount.php'>Skapa konto</a>
                    <a href='support.php'>Support</a>
                ";
            }
        ?>
    </div>
    <div id="handle">Menu</div>
</nav>
