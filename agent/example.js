console.log("Hello, World!");

let res = await fetch("https://api.github.com/repos/Ranzeplay/saysth/issues");

if (res.ok) {
    let data = await res.json();
    console.log(data);
} else {
    console.error("This is an error");
}