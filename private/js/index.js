$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    ajax("get", "/api/announcement", undefined, () => $("#announcement").html("数据库错误"), dat => {
        let announcementTemplate = juicer.compile($("#announcement-template").html());

        for (let i of dat) {
            textRenderer(i.content).then(content =>
                $("#announcement").append(announcementTemplate.render({
                    aid: i.aid,
                    title: i.title,
                    content: content
                }))
            )
        }
    });
});