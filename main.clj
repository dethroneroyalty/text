@1
;;; the beauty of clujure if that IF EVEN you put atom/ref in object
;; then it ONLY if you REALLY need this shared state, and by mutating
;; value you now you don't accidantelly mutate another object behavior
;; just because it share this state .... BUT DO IT ON PURPOSE ... since
;; thervise you would just use immutable-data(aka)explict-returning.
