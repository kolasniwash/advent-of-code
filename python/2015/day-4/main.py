import hashlib
EXAMPLE = 'abcdef' #'609043'
PUZZLE_INPUT = 'yzbqklnj'


def main(key):
    index = 0
    test_hash = '100000'
    while test_hash[:5] != '00000':
        index += 1
        print(index)
        secret_key = key + str(index)
        test_hash = hashlib.md5(str.encode(secret_key)).hexdigest()
    return test_hash, index
if __name__ == '__main__':
    output_hash, number = main(PUZZLE_INPUT)
    print(output_hash, number)
    #282749
    #9962624