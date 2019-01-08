<?php
    session_start();
    session_destroy();
    session_regenerate_id(true);
    echo "<script>window.location.replace('http://192.168.30.12/~attend/');</script>";
?>
