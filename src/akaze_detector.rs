use akaze::Akaze;
use arrsac::Arrsac;
use bitarray::BitArray;
use cv::knn::*;
use cv_core::nalgebra::{Point2, Vector2};
use cv_core::sample_consensus::Consensus;
use cv_core::{CameraModel, FeatureMatch};
use rand::SeedableRng;
// use rand_pcg::Pcg64;
use std::path::Path;

pub fn akaze_detect() {
    // Load the image.
    let src_image_a = image::open("res/0000000000.png").expect("failed to open image file");
    let src_image_b = image::open("res/0000000014.png").expect("failed to open image file");

    // Create an instance of `Akaze` with the default settings.
    let akaze = Akaze::default();

    // Extract the features from the image using akaze.
    let (key_points_a, descriptors_a) = akaze.extract(&src_image_a);
    let (key_points_b, descriptors_b) = akaze.extract(&src_image_b);
    let matches = symmetric_matching(&descriptors_a, &descriptors_b);
}

// NOTE: these are lifted from the Akaze demo
/// This function performs non-symmetric matching from a to b.
fn matching(a_descriptors: &[BitArray<64>], b_descriptors: &[BitArray<64>]) -> Vec<Option<usize>> {
    let knn_b = LinearKnn {
        metric: Hamming,
        iter: b_descriptors.iter(),
    };
    (0..a_descriptors.len())
        .map(|a_feature| {
            let knn = knn_b.knn(&a_descriptors[a_feature], 2);
            if knn[0].distance + 24 < knn[1].distance {
                Some(knn[0].index)
            } else {
                None
            }
        })
        .collect()
}

/// This function performs symmetric matching between `a` and `b`.
///
/// Symmetric matching requires a feature in `b` to be the best match for a feature in `a`
/// and for the same feature in `a` to be the best match for the same feature in `b`.
/// The feature that a feature matches to in one direction might not be reciprocated.
/// Consider a 1d line. Three features are in a line `X`, `Y`, and `Z` like `X---Y-Z`.
/// `Y` is closer to `Z` than to `X`. The closest match to `X` is `Y`, but the closest
/// match to `Y` is `Z`. Therefore `X` and `Y` do not match symmetrically. However,
/// `Y` and `Z` do form a symmetric match, because the closest point to `Y` is `Z`
/// and the closest point to `Z` is `Y`.
///
/// Symmetric matching is very important for our purposes and gives stronger matches.
fn symmetric_matching(a: &[BitArray<64>], b: &[BitArray<64>]) -> Vec<[usize; 2]> {
    // The best match for each feature in frame a to frame b's features.
    let forward_matches = matching(a, b);
    // The best match for each feature in frame b to frame a's features.
    let reverse_matches = matching(b, a);
    forward_matches
        .into_iter()
        .enumerate()
        .filter_map(move |(aix, bix)| {
            // First we only proceed if there was a sufficient bix match.
            // Filter out matches which are not symmetric.
            // Symmetric is defined as the best and sufficient match of a being b,
            // and likewise the best and sufficient match of b being a.
            bix.map(|bix| [aix, bix])
                .filter(|&[aix, bix]| reverse_matches[bix] == Some(aix))
        })
        .collect()
}
