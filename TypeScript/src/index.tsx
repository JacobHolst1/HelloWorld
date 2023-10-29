
type card = {
    id: number
    title?: string
    category?:string
    created: Date
    noteText?: string
}

let cardList = []

//Create a new card div
function createCard() {
    let a = {
        id:1,
        title: document.getElementById("headerBox")?.textContent,
        category: "general",
        created: new Date(),
        noteText: document.getElementById("textBox")?.textContent
    }
    cardList.push(a)
    let newDiv = document.createElement("div");
    newDiv.textContent = a.title + "";
    newDiv.setAttribute("class", "noteCard");
    document.body.appendChild(newDiv);

    var q = document.getElementById("taskbar")
    q!.innerHTML = "Success!"
    return (
        <div id="{toString(a.id)}" class="noteCard">
            <p>{a.title}</p>
            <p>This is a new card!</p>
            <p>{a.noteText}</p>
        </div>
    )
}

// Add an event listener for the above function
document.getElementById("submit")!.addEventListener("click", createCard)
