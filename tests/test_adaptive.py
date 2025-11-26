from ars.adaptive.ars_adaptive import ARSAdaptiveSort

def test_adaptive_sort_basic():
    sorter = ARSAdaptiveSort()
    assert sorter.sort([5, 4, 3]) == [3, 4, 5]
