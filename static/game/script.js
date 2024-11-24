fetch("/api/v1/games")
    .then((response) => response.json())
    .then((json) => console.log(json))
fetch("/api/v1/games", {
    method: "POST",
    body: JSON.stringify({
        name: "Test",
        difficulty: "easy",
        board: [
                    ["X", "X", "X", "X", "X", "X", "", "", "", "", "", "", "", "", ""],
                    ["O", "O", "O", "O", "O", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                    ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
                ]
    }),
    headers: {
        "Content-type": "application/json; charset=UTF-8"
    }
}).then((response) => response.json())
.then((json) => console.log(json));