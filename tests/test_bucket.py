from ars.bucket.ars_bucket import ARSBucketSort

def test_bucket_sort_basic():
    sorter = ARSBucketSort()
    assert sorter.sort([3.5, 1.2, 2.8]) == [1.2, 2.8, 3.5]
