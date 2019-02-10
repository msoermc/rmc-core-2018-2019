let gamepad;
let connectionStatus;

window.addEventListener("gamepadconnected", function (e) {
    gamepad = navigator.getGamepads()[e.gamepad.index];
    connectionStatus = document.getElementById("connectionStatus");
    console.log("Gamepad connected at index %d: %s. %d buttons, %d axes.",
        e.gamepad.index, e.gamepad.id,
        e.gamepad.buttons.length, e.gamepad.axes.length);

    connectionStatus.innerHTML = "Connected";
    connectionStatus.style.color = "black";
    update();
});

window.addEventListener("gamepaddisconnected", function () {
    console.log("Gamepad disconnected");
    connectionStatus.innerHTML = "Disconnected";
    connectionStatus.style.color = "red";
    cancelAnimationFrame(update);
});

function update() {
    draw_axis("movement-axis", gamepad.axes[0] * 45 + 50, gamepad.axes[1] * 45 + 50);
    draw_axis("camera-axis", gamepad.axes[2] * 45 + 50, gamepad.axes[3] * 45 + 50);

    draw_button("a", gamepad.buttons[0].pressed);
    draw_button("b", gamepad.buttons[1].pressed);
    draw_button("x", gamepad.buttons[2].pressed);
    draw_button("y", gamepad.buttons[3].pressed);

    draw_button("lb", gamepad.buttons[4].pressed);
    draw_button("rb", gamepad.buttons[5].pressed);
    draw_button("lt", gamepad.buttons[6].pressed);
    draw_button("rt", gamepad.buttons[7].pressed);

    draw_button("back", gamepad.buttons[8].pressed);
    draw_button("start", gamepad.buttons[9].pressed);
    draw_button("ljoy", gamepad.buttons[10].pressed);
    draw_button("rjoy", gamepad.buttons[11].pressed);

    draw_button("up", gamepad.buttons[12].pressed);
    draw_button("down", gamepad.buttons[13].pressed);
    draw_button("left", gamepad.buttons[14].pressed);
    draw_button("right", gamepad.buttons[15].pressed);

    requestAnimationFrame(update);
}

function draw_axis(axis, x, y) {
    let canvas = document.getElementById(axis);
    let width = canvas.width;
    let height = canvas.height;
    let context = canvas.getContext("2d");
    context.clearRect(0, 0, width, height);
    context.beginPath();
    context.moveTo(x - 5, y);
    context.lineTo(x + 5, y);
    context.moveTo(x, y - 5);
    context.lineTo(x, y + 5);
    context.stroke();
}

function draw_button(name, state) {
    let data = document.getElementById(name);
    if (state) {
        data.style.backgroundColor = "red";
    } else {
        data.style.backgroundColor = "white";
    }
}