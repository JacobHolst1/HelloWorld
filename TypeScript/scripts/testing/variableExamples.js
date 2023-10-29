"use strict";
// import * as fs from "fs";
// Declare a mutable number
var number = 1.888;
// Declare an immutable string
const word = "Today's Accomplishments:";
// Declare a tuple for numerical values of the week
let dayOfWeek = ["Sunday", 1];
let OctNum = 10 /* monthByName.October */;
//Declare a class for a note/entry
let entryObj = { id: 1, title: "Untitled", date: Date() };
// Declare a function for translating date from a number to a month
function numToDate(num) {
    if (typeof num === 'number') {
    }
}
// Function for returning a title if an entry has one
function getTitle(id) {
    return id === 0 ? null : { words: "Today's achievements:", date: new Date() };
}
let firstTitle = getTitle(0);
console.log(firstTitle === null || firstTitle === void 0 ? void 0 : firstTitle.date.getFullYear());
console.log(firstTitle === null || firstTitle === void 0 ? void 0 : firstTitle.words);
//open a file using fs
// const oldWords = fs.readFileSync("./words.txt", "utf-8");
//# sourceMappingURL=variableExamples.js.map