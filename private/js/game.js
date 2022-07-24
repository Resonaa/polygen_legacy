$(() => {
    let roomTemplate = juicer.compile($("#room-template").html());

    $("#ongoing-games").html("");

    function addRoom(room) {
        let players = `${room.players.length}玩家: `;

        for (let player of room.players) {
            players += userLink(player) + ", ";
        }

        $("#ongoing-games").append(roomTemplate.render({
            rid: room.rid,
            status: room.status,
            config: room.config,
            players: players.substring(0, players.length - 2)
        }));
    }

    addRoom({
        rid: 1,
        status: "error",
        config: {
            mode: "六边形",
            map: "随机地图"
        },
        players: ["jwcub"]
    });
    addRoom({
        rid: 2,
        status: "ongoing",
        config: {
            mode: "四边形",
            map: "空白地图"
        },
        players: ["jwcub"]
    });

    $("[data-bs-toggle='tooltip']").each((_, e) => new bootstrap.Tooltip(e));
});