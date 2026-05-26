use std::{env, time::Duration};

use criterion::Criterion;

#[derive(Debug, Clone, Copy)]
pub(crate) enum BenchProfile {
    Quick,
    Report,
}

impl BenchProfile {
    #[inline]
    fn current() -> Self {
        match env::var("BENCH_PROFILE").as_deref() {
            Ok("report") => Self::Report,
            Ok("quick") | Err(_) => Self::Quick,
            Ok(other) => panic!("invalid BENCH_PROFILE={other:?}; expected `quick` or `report`"),
        }
    }

    #[inline]
    fn baseline(self) -> String {
        match self {
            Self::Quick => "quick".into(),
            Self::Report => "report".into(),
        }
    }

    #[inline]
    fn criterion(self) -> Criterion {
        match self {
            // 本地快速验证：保持短反馈周期。
            Self::Quick => Criterion::default()
                .sample_size(20)
                .warm_up_time(Duration::from_millis(100))
                .measurement_time(Duration::from_millis(300))
                .nresamples(10_000)
                .without_plots()
                .save_baseline(self.baseline()),

            // 可发布报告：适当拉长，覆盖集合构造与集合代数的稳定统计需求。
            Self::Report => Criterion::default()
                .sample_size(60)
                .warm_up_time(Duration::from_secs(1))
                .measurement_time(Duration::from_secs(2))
                .nresamples(20_000)
                .save_baseline(self.baseline()),
        }
    }
}

/// Shared Criterion configuration.
///
/// `BENCH_PROFILE=report` generates the publishable HTML report.
/// Missing `BENCH_PROFILE` defaults to fast local feedback.
#[inline]
pub(crate) fn config() -> Criterion {
    BenchProfile::current().criterion()
}
