$(() => {
    function changeCaptcha() {
        $("#captcha")[0].value = "";
        $(".captcha")[0].src = "/api/captcha?t=" + Math.random();
    }

    $(".captcha").click(changeCaptcha);

    $("#submit").click(() => {
        $(".spinner-border").show();
        $("#submit").attr("disabled", 0);

        ajax("post", "/login", { "username": $("#username")[0].value, "password": $("#password")[0].value, "captcha": $("#captcha")[0].value }, msg => {
            swal("登录失败", msg, "error");
            $(".spinner-border").hide();
            $("#submit").removeAttr("disabled");
            changeCaptcha();
        }, () => window.location.href = '/');
    });

    $("#login").keydown(e => {
        let keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $("#submit").click();
        }
    });
});