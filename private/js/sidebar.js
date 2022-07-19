$(() => {
    ajax("get", "/api/announcement", undefined, () => $("#announcement").html("数据库错误"), dat => {
        let announcementTemplate = juicer.compile($("#announcement-template").html());

        $("#announcement").html("");

        for (let i of dat) {
            $("#announcement").append(announcementTemplate.render({
                aid: i.aid,
                title: i.title,
                content: textRenderer(i.content)
            }));
        }
    });
});