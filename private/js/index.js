$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    ajax("get", "/api/announcement", undefined, () => $("#announcement").html("数据库错误"), async (dat) => {
        let announcementTemplate = juicer.compile($("#announcement-template").html());

        for (let i of dat) {
            $("#announcement").append(announcementTemplate.render({
                aid: i.aid,
                title: i.title,
                content: await textRenderer(i.content)
            }));
        }
    });

    let pid = 0;

    $(".labeled.primary.button").click(() => {
        disableButton(".labeled.primary.button");
        let content = vditor.getValue();

        textRenderer(content).then(html => {
            if (!$("<div/>").html(html).text().trim()) {
                $("body").toast({
                    class: "error",
                    title: "发送失败",
                    message: "内容不能为空"
                });
                enableButton(".labeled.primary.button");
            }
            else {
                ajax("post", "/api/post", content, msg => {
                    $("body").toast({
                        class: "error",
                        title: "发送失败",
                        message: msg
                    });
                    enableButton(".labeled.primary.button");
                }, () => {
                    vditor.setValue("");
                    window.location.reload();
                });
            }
        });
    });

    let page = 1;
    let postTemplate = juicer.compile($("#post-template").html());
    let noMore = false;

    function getAuthor(post) {
        return userLink(post.author);
    }

    function addPost() {
        if (noMore) {
            return;
        }

        ajax("get", `/api/post?`, { parent: pid, page: page }, undefined, async (dat) => {
            for (let i of dat) {
                $(".feed").append(postTemplate.render({
                    realTime: i.time,
                    deltaTime: deltaTime(i.time),
                    pid: i.pid,
                    commentAmount: 0,
                    content: await textRenderer(i.content),
                    author: getAuthor(i),
                    home: true,
                }));
            }

            if (dat.length < 10) {
                noMore = true;
            }

            Vditor.mathRender(document);
            Vditor.highlightRender(document);
        });
    }

    addPost();

    $(window).scroll(() => {
        let scrollTop = $(this).scrollTop();
        let scrollHeight = $(document).height();
        let windowHeight = $(this).height();

        if (scrollHeight - scrollTop - windowHeight <= 10) {
            page++;
            addPost();
        }
    });
});