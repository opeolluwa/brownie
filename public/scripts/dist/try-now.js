"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
//reference the elements in the DOM
const tryNowForm = document.querySelector("#try-now-form");
const inputFeed = document.querySelector("#try-now-form input");
const tryNowButton = document.querySelector("#try-now-form button");
//add event listener to the button
tryNowForm === null || tryNowForm === void 0 ? void 0 : tryNowForm.addEventListener("submit", function (event) {
    var _a;
    return __awaiter(this, void 0, void 0, function* () {
        event.preventDefault();
        // const inputValue = inputFeed?.value; //get the value of the input
        const validInput = validateInput(inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.value);
        // alert(validInput)
        if (!validInput) {
            (_a = document === null || document === void 0 ? void 0 : document.querySelector("#try-now-form span.error-message")) === null || _a === void 0 ? void 0 : _a.classList.add("show-error");
        }
    });
});
//if no input or invalid input
function validateInput(inputValue) {
    if (!inputValue.trim().startsWith("http") || !inputValue.trim().startsWith("https://")) {
        return false;
    }
    return true;
}
//hide the error message on innput focus{
inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.addEventListener("input", function () {
    var _a;
    (_a = document === null || document === void 0 ? void 0 : document.querySelector("#try-now-form span.error-message")) === null || _a === void 0 ? void 0 : _a.classList.remove("show-error");
});
function hideError(domSelector) {
    const error = document.querySelector(domSelector);
    error.style.display = "none";
}
//define a function to show error on the input
function showError(input, message) {
    const formControl = input.parentElement;
    const errorMessage = formControl.querySelector("span.error-message");
    errorMessage.innerText = message;
    errorMessage.classList.add("show");
}
