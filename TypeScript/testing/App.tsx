

// DISCLAIMER NOTE:
// Not actually part of the program, I kept this in for self-learning purposes

type noteType = {
    id: number;
    title?: string;
    noteText: string;
    createAt?: Date;   
}


const myNotesList = [
    {id:1, noteText:"This is some text"},
    {id:2, noteText:"This is also some text"},
    {id:3, noteText:"This is another box of text"}
]

function App() {
    // const [notesList, setNotesList] = useState(myNotesList as noteType[]);
    return (
        <div className= "App" >
        {/* <TailWindLayout1 title = "" >
            <CreateNoteCard />
            < div >
            <postcard3/>
            <postcard3/>
            <postcard3/>
            < postcard3 />
        {
            myNotesList.map(val, key)
                < div key = { key } >
                    <postcard3 note={val}/>
                </div>
        }
            </TailWindLayout1> */}
        </div>
        
    )
}