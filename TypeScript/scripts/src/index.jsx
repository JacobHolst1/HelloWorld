"use strict";
let cardList = [];
//Create a new card div
function createCard() {
    var _a, _b;
    let a = {
        id: 1,
        title: (_a = document.getElementById("headerBox")) === null || _a === void 0 ? void 0 : _a.textContent,
        category: "general",
        created: new Date(),
        noteText: (_b = document.getElementById("textBox")) === null || _b === void 0 ? void 0 : _b.textContent
    };
    cardList.push(a);
    let newDiv = document.createElement("div");
    newDiv.textContent = a.title + "";
    newDiv.setAttribute("class", "noteCard");
    document.body.appendChild(newDiv);
    var q = document.getElementById("taskbar");
    q.innerHTML = "Success!";
    return (<div id="{toString(a.id)}" class="noteCard">
            <p>{a.title}</p>
            <p>This is a new card!</p>
            <p>{a.noteText}</p>
        </div>);
}
// Add an event listener for the above function
document.getElementById("submit").addEventListener("click", createCard);
//# sourceMappingURL=index.jsx.map