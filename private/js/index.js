$(() => {
    let pid = 0;
    let s = window.location.href.match(/\d+$/);
    if (s != null) {
        pid = Number(s[0]);
    }

    let postEditor = new MarkdownPalettes("#post-editor");

    $("#post").click(() => {
        if (postEditor.content && postEditor.content.trim()) {
            ajax("post", "/api/post", { content: postEditor.content, parent: pid }, msg => swal("发送失败", msg, "error"), () => window.location.reload())
        } else {
            swal("发送失败", "内容不能为空", "error");
        }
    });

    let page = 1;
    let postTemplate = Handlebars.compile(`
        <article class="polygen-item post">
            <header class="post-hd">
                <div class="post-meta">
                    <div class="post-author"></div>
                    <span class="post-time">&nbsp;<time title="{{realTime}}">{{deltaTime}}</time>&nbsp;{{commentAmount}}评论</span>
                </div>
            </header>

            <a href="/post/{{pid}}" style="color: unset;"><div class="post-content"></div></a>
        </article>`);

    function getAuthor(post) {
        if (post.parent == 0) {
            return userLink(post.author);
        }

        return `${userLink(post.author)} <a href="/post/${post.parent}"><i class="far fa-long-arrow-alt-right"></i> ${userLink(ajaxSync("get", "/api/post", { pid: post.parent }).msg.author)}`;
    }

    if (pid != 0) {
        let dat = ajaxSync("get", `/api/post?pid=${pid}`, undefined).msg;
        $("#main-part").prepend(postTemplate({ realTime: dat.time, deltaTime: deltaTime(dat.time), commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: dat.pid }).msg }));
        $(".post-content").last().html(textRenderer(dat.content)).css("max-height", "none").css("overflow-y", "hidden")[0].parentElement.attributes.removeNamedItem("href");
        $(".post-author").last().html(getAuthor(i));
        document.title = `${dat.author}的说说 - polygen`;
    }

    let view = "tree";

    function addPost() {
        if ($(".loader").is(":hidden")) {
            return;
        }

        $(".loader").html(`<div class="spinner-border"></div>`);

        ajax("get", `/api/post?`, { parent: pid, page: page, view: view }, undefined, dat => {
            for (let i of dat) {
                $("#load-more").before(postTemplate({ realTime: i.time, deltaTime: deltaTime(i.time), pid: i.pid, commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: i.pid }).msg }));
                $(".post-content").last().html(textRenderer(i.content));
                $(".post-author").last().html(getAuthor(i));
                $(".post").last().addClass("comment");
            }

            if (dat.length < 10) {
                $(".loader").after(`<span id="no-more">没有更多了</span>`).hide();
            } else {
                $(".loader").html("点击查看更多...");
            }
        });
    }

    addPost();

    $("#load-more").click(() => {
        page++;
        addPost();
    });

    $(".post-selector > .nav-link").click((e) => {
        $(".post-selector > .active").removeClass("active");
        e.target.classList.add("active");
        view = e.target.id;
        page = 1;
        $(".comment").remove();
        $(".loader").show();
        $("#no-more").remove();
        addPost();
    });
});