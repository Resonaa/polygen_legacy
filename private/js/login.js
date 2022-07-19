$(() => {
    function changeCaptcha() {
        $("#captcha").val("");
        $(".captcha")[0].src = "/api/captcha?t=" + Math.random();
    }

    $(".captcha").click(changeCaptcha);

    $("#submit").click(() => {
        $(".spinner-border").show();
        $("#submit").attr("disabled", 0);

        ajax("post", "/login", { username: $("#username").val(), password: $("#password").val(), captcha: $("#captcha").val() }, msg => {
            swal("登录失败", msg, "error");
            $(".spinner-border").hide();
            $("#submit").removeAttr("disabled");
            changeCaptcha();
        }, () => window.location.href = "/");
    });

    $(".auth-form").keydown(e => {
        let keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $("#submit").click();
        }
    });
});