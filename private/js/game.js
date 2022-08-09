$(() => {
    let roomTemplate = juicer.compile($("#room-template").html());

    let translation = {
        Quadrilateral: "四边形", Hexagon: "六边形", Random: "随机地图",
    };

    function addRoom(room) {
        let players = `${room.players.length}玩家: `;

        for (let player of room.players) {
            players += userLink(player) + ", ";
        }

        $("#ongoing-games").append(roomTemplate.render({
            rid: room.rid,
            status: room.status.toLowerCase(),
            mode: translation[room.map.config.mode],
            map: translation[room.map.config.tp],
            players: players.substring(0, players.length - 2)
        }));
    }

    ajax("get", "/game/list", undefined, undefined, rooms => {
        $("#ongoing-games").html("");

        for (let room of rooms) {
            addRoom(room);
        }

        $("[data-bs-toggle='tooltip']").each((_, e) => new bootstrap.Tooltip(e));
    });
});