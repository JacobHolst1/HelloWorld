// import * as fs from "fs";


// Declare a mutable number
var number = 1.888
// Declare an immutable string
const word = "Today's Accomplishments:"
// Declare a tuple for numerical values of the week
let dayOfWeek: [string, number] = ["Sunday", 1]
// Declare an enum mapping months of the year to a numerical value
const enum monthByName {"January" = 1, "February" = 2, "March" = 3, "April" = 4, "May" = 5, "June" = 6, "July" = 7, "August" = 8, "September" = 9, "October" = 10, "November" = 11, "December" = 12 }
let OctNum = monthByName.October

//Declare a class for a note/entry
let entryObj: {
    id: number
    title?: string
    date: string
} = { id: 1, title: "Untitled", date:Date()}

// Declare a function for translating date from a number to a month
function numToDate(num: number | string) {
    if (typeof num === 'number') {
        
    }
}

type entry = {
    words: string
    date: Date
}

// Function for returning a title if an entry has one
function getTitle(id: number): entry | null {
    return id === 0 ? null : { words: "Today's achievements:", date:new Date()};
}

let firstTitle = getTitle(0);

console.log(firstTitle?.date.getFullYear());
console.log(firstTitle?.words);

//open a file using fs
// const oldWords = fs.readFileSync("./words.txt", "utf-8");