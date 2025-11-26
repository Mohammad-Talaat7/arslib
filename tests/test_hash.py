from ars.hash.ars_hash import ARSHashSort

def test_hash_sort_basic():
    sorter = ARSHashSort()
    assert sorter.sort([3, 1, 2]) == [1, 2, 3]

