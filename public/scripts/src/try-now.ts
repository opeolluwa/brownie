//reference the elements in the DOM
const tryNowForm = document.querySelector("#try-now-form");
const inputFeed: any = document.querySelector("#try-now-form input");
const tryNowButton: any = document.querySelector("#try-now-form button");


//add event listener to the button
tryNowForm?.addEventListener("submit", async function (event) {
    event.preventDefault();//prevent auto submission
    const inputValue = inputFeed?.value; //get the value of the input
    const validInput: boolean = validateInput(inputFeed?.value)
    if (!validInput) {
        appendStyle("d-inline-block")
    }
    tryNowButton.disabled = true;//disable the button
    tryNowButton.innerHTML = "Loading...";//change the button text
    // disable the button, hide the error message and send the content to the server
    // document?.querySelector("#try-now-form button").classList.add("disabled") = true;
    const data = {
        raw_url: inputValue
    };
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    };
    // console.log(options);
    const serverResponse: any = await fetch("/api/v1/links/minify", options);
    // const responseData: any = await serverResponse;
    //det the button to active 
    tryNowButton.disabled = false;
    tryNowButton.innerHTML = "Try Now 🚀";
    //clear the input field
    inputFeed.value = "";
    // console.log(serverResponse.status);
    // console.log(Object.keys(responseData.data));
    console.log(serverResponse.body);
    
});

//hide the error message on innput focus{
inputFeed?.addEventListener("input", function () {
    // document?.querySelector("#try-now-form span.error-message")?.classList.remove("show-error")
    removeStyle("d-inline-block")
});


//if no input or invalid input
function validateInput(inputValue: string): boolean {
    if (!inputValue.trim().startsWith("http") || !inputValue.trim().startsWith("https://")) {
        return false
    }
    return true
}
function removeStyle(className: string, domSelector: string = "#try-now-form span.error-message",) {
    document?.querySelector(domSelector)?.classList.remove(className)
}
function appendStyle(className: string, domSelector: string = "#try-now-form span.error-message",) {
    document?.querySelector(domSelector)?.classList.add(className)
}