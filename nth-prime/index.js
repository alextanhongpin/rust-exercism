function main () {
  let test = [2, 3, 5, 7, 9, 12]
  console.log(test.map(isPrime))
  console.log(nthPrime(10001))
}

function nthPrime (n) {
  if (n <= 0) {
    return 0
  }
  let count = 0
  let i = 1
  while (count !== n) {
    i += 1
    if (isPrime(i)) {
      count += 1
    }
  }
  return i
}

function isPrime (n) {
  let upper = Math.floor(Math.sqrt(n)) - 1
  return !Array(upper).fill(2).map((n, i) => i + n).some(i => n % i === 0)
}

main()
