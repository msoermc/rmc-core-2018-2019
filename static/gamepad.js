var gamepad;
var connectionStatus;

window.addEventListener("gamepadconnected", function(e) {
    gamepad = navigator.getGamepads()[e.gamepad.index];
    connectionStatus = document.getElementById("connectionStatus");
    console.log("Gamepad connected at index %d: %s. %d buttons, %d axes.",
        e.gamepad.index, e.gamepad.id,
        e.gamepad.buttons.length, e.gamepad.axes.length);

    connectionStatus.innerHTML = "Connected";
    connectionStatus.style.color = "black";
    update();
});

window.addEventListener("gamepaddisconnected", function() {
    console.log("Gamepad disconnected");
    connectionStatus.innerHTML = "Disconnected";
    connectionStatus.style.color = "red";
    cancelAnimationFrame(update);
});

function update() {
    draw_axis("movement-axis", gamepad.axes[0]*45 + 50, gamepad.axes[1]*45 + 50);
    draw_axis("camera-axis", gamepad.axes[2]*45 + 50, gamepad.axes[3]*45 + 50);

    requestAnimationFrame(update);
}

function draw_axis(axis, x, y) {
    let canvas = document.getElementById(axis);
    let width = canvas.width;
    let height = canvas.height;
    let context = canvas.getContext("2d");
    context.clearRect(0, 0, width, height);
    context.beginPath();
    context.moveTo(x-5, y);
    context.lineTo(x+5, y);
    context.moveTo(x, y-5);
    context.lineTo(x, y+5);
    context.stroke();
}