//reference the elements in the DOM
const tryNowForm = document.querySelector("#try-now-form");
const inputFeed: any = document.querySelector("#try-now-form input");
const tryNowButton = document.querySelector("#try-now-form button");


//add event listener to the button
tryNowButton?.addEventListener("click", async function (event) {
    event.preventDefault();
    const inputValue = inputFeed?.value; //get the value of the input
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
    const serverResponse: any = await fetch("/api/minify", options);
    const responseData: any = await serverResponse.json();
    if (responseData.error) {
        showError(inputFeed, responseData.error);
        return;
    }
    //log the response
    console.log(responseData.minified_url);
    alert(inputValue);
});

//some update
//hide the error message on innput focus{
inputFeed?.addEventListener("input", function () {
    hideError("#try-now-form .error-message");
});



function hideError(domSelector: string) {
    const error: any = document.querySelector(domSelector);
    error.style.display = "none";
}

//define a function to show error on the input
function showError(input: any, message: string) {
    const formControl = input.parentElement;
    const errorMessage = formControl.querySelector("span.error-message");
    errorMessage.innerText = message;
    errorMessage.classList.add("show");

}