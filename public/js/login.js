$(() => {
    $("#submit").click(() => {
        $.ajax({
            type: "post",
            url: "/login",
            data: JSON.stringify({ "username": $("#username")[0].value, "password": $("#password")[0].value }),
            dataType: "json",
            success: function (res) {
                if (res.status == "success") {
                    toast("success", "登录成功", "欢迎", 700, () => window.location.href = '/');
                } else {
                    toast("error", "登录失败", res.msg);
                }
            }
        });
    });
    $(document).keydown(e => {
        let keyCode = e.which || e.keyCode;
        if (keyCode == 13) {
            $("#submit").click();
        }
    });
})