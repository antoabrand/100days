const testCollection = [12,33,5321,4,22,89,88000,5421,2345];

binarySearch(testCollection);

function binarySearch(collection, testNum){

    const sortedCollection = collection.sort((a,b) => a-b); 

    let top = sortedCollection[collection.length() - 1]; 
    let bottom = sortedCollection[0]; 
    let mid = Math.floor(sortedCollection.length / 2); 
    let found = 0;

    while(found === 0){

        if(sortedCollection[mid] === testNum){
            found = sortedCollection[mid];
        }
    }

    console.log("Found: " + testNum + " in index" + mid);
    sortedCollection.forEach(x => console.log(x));
}

function calculateMid(){
    return newMid;
}

