$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    function changeCaptcha() {
        $("#captcha").val("");
        $(".captcha")[0].src = "/api/captcha?t=" + Math.random();
    }

    $(".captcha").click(changeCaptcha);

    $(".submit").click(() => {
        disableButton(".submit");

        ajax("post", "/login", { username: $("#username").val(), password: $("#password").val(), captcha: $("#captcha").val() }, msg => {
            toast("error", "登陆失败", msg);
            enableButton(".submit");
            changeCaptcha();
        }, () => window.location.href = "/");
    });

    $("form").keydown(e => {
        const keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $(".submit").click();
        }
    });
});