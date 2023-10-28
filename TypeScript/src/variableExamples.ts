
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
let entry: {
    id: number
    title?: string
    date?: Date
} = { id: 1, title: "Untitled" }

// Declare a function for translating date from a number to a month
function numToDate(num: number | string) {
    if (typeof num === 'number') {
        
    }
}