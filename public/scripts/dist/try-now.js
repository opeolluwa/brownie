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
    return __awaiter(this, void 0, void 0, function* () {
        event.preventDefault(); //prevent auto submission
        const inputValue = inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.value; //get the value of the input
        const validInput = validateInput(inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.value);
        if (!validInput) {
            appendStyle("d-inline-block");
        }
        // disable the button, hide the error message and send the content to the server
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
        console.log(options);
        // const serverResponse: any = await fetch("/api/minify", options);
        // const responseData: any = await serverResponse.json();
        /* if (responseData.error) {
            appendStyle(inputFeed, responseData.error);
            return;
        } */
        //log the response
        // console.log(responseData.minified_url);
        // alert(inputValue);
    });
});
//hide the error message on innput focus{
inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.addEventListener("input", function () {
    // document?.querySelector("#try-now-form span.error-message")?.classList.remove("show-error")
    removeStyle("d-inline-block");
});
//if no input or invalid input
function validateInput(inputValue) {
    if (!inputValue.trim().startsWith("http") || !inputValue.trim().startsWith("https://")) {
        return false;
    }
    return true;
}
function removeStyle(className, domSelector = "#try-now-form span.error-message") {
    var _a;
    (_a = document === null || document === void 0 ? void 0 : document.querySelector(domSelector)) === null || _a === void 0 ? void 0 : _a.classList.remove(className);
}
function appendStyle(className, domSelector = "#try-now-form span.error-message") {
    var _a;
    (_a = document === null || document === void 0 ? void 0 : document.querySelector(domSelector)) === null || _a === void 0 ? void 0 : _a.classList.add(className);
}
