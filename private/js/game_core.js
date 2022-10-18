const ws = new WebSocket(`ws://${window.location.hostname}:7878`);

ws.onopen = () => {
    console.log("Connection open");
    ws.send(JSON.stringify({ name: "Identify", dat: { username: username, identity: identity } }));
};

ws.onmessage = evt => {
    let dat = JSON.parse(evt.data);
    console.log("Received Message:", dat);
};

ws.onclose = () => {
    console.log("Connection closed");
    window.location.href = "/game";
};

window.game = new CatchTheCatGame({
    w: 19,
    h: 19,
    r: 15,
    backgroundColor: 16777215,
    parent: "catch-the-cat",
    statusBarAlign: "center",
    credit: "polygen",
});