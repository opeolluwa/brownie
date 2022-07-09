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
tryNowButton === null || tryNowButton === void 0 ? void 0 : tryNowButton.addEventListener("click", function (event) {
    return __awaiter(this, void 0, void 0, function* () {
        event.preventDefault();
        const inputValue = inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.value; //get the value of the input
        if (!inputValue || !inputValue.trim().startsWith("http") || !inputValue.trim().startsWith("https://")) {
            showError(inputFeed, "Please enter a valid url starting with http:// or https://");
            return;
        }
        //hide the error message and send the content to the server
        hideError("#try-now-form .error-message");
        const data = {
            url: inputValue
        };
        const options = {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        };
        const serverResponse = yield fetch("/api/minify", options);
        const responseData = yield serverResponse.json();
        if (responseData.error) {
            showError(inputFeed, responseData.error);
            return;
        }
        //log the response
        console.log(responseData.minified_url);
        alert(inputValue);
    });
});
//hide the error message on innput focus{
inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.addEventListener("input", function () {
    hideError("#try-now-form .error-message");
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
