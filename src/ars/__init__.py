"""ARS (Adjacency Run Sort).

This package contains three variants of the ARS algorithm:
- ARSHashSort: Integer hash-based version
- ARSBucketSort: Float bucket-based version
- ARSAdaptiveSort: Adaptive BST/Skip-list version
"""

from .adaptive.ars_adaptive import ARSAdaptiveSort
from .bucket.ars_bucket import ARSBucketSort

# Public API imports (empty skeletons for Phase 1)
from .hash.ars_hash import ARSHashSort
from .version import __version__

__all__ = [
    "ARSHashSort",
    "ARSBucketSort",
    "ARSAdaptiveSort",
    "__version__",
]
