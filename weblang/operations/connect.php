<?php
    function connect($server_name, $database_name, $username, $password) {
        try {
            $conn = new PDO("mysql:host=$server_name;dbname=$database_name", $username, $password);
        $conn->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
        } catch(PDOException $e) {
            echo "\nError: Connection failed: " .$e->getMessage();
        }

        return $conn;
    }
?>
